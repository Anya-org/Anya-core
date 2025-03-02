import 'dart:async';
import 'dart:convert';
import 'package:logging/logging.dart';
import 'package:web5_dart/web5_dart.dart' as web5;
import 'package:bitcoin_anchoring/bitcoin_anchoring.dart';

import 'metrics.dart';
import 'read_first_dwn.dart';
import 'identity.dart';

/// Service class for Web5 functionality
class Web5Service {
  final Logger _logger = Logger('Web5Service');
  final web5.Web5Client _web5;
  final web5.DID _did;

  Web5Service._(this._web5, this._did);

  /// Connect to Web5 and create a new instance
  static Future<Web5Service> connect() async {
    try {
      final client = web5.Web5Client();
      final did = await web5.DID.create();
      
      return Web5Service._(client, did);
    } catch (e) {
      throw Web5Exception('Failed to connect to Web5: $e');
    }
  }

  /// Get the DID associated with this Web5 instance
  String get did => _did.toString();

  /// Create a new record in the DWN
  Future<web5.Record> createRecord({
    required String collection,
    required Map<String, dynamic> data,
    String? schema,
    List<String>? recipients,
  }) async {
    try {
      final record = await _web5.dwn.records.create(
        web5.CreateRecordOptions(
          data: jsonEncode(data),
          dataFormat: 'application/json',
          schema: schema,
          recipient: recipients?.first,
          published: true,
        ),
      );

      _logger.info('Created record: ${record.id}');
      return record;
    } catch (e) {
      _logger.severe('Failed to create record: $e');
      throw Web5Exception('Failed to create record: $e');
    }
  }

  /// Query records from the DWN
  Future<List<web5.Record>> queryRecords({
    required String collection,
    Map<String, dynamic>? filter,
    String? schema,
  }) async {
    try {
      final records = await _web5.dwn.records.query(
        web5.QueryRecordOptions(
          filter: filter,
          schema: schema,
        ),
      );

      _logger.info('Queried ${records.length} records');
      return records;
    } catch (e) {
      _logger.severe('Failed to query records: $e');
      throw Web5Exception('Failed to query records: $e');
    }
  }

  /// Update an existing record
  Future<web5.Record> updateRecord({
    required String recordId,
    required Map<String, dynamic> data,
  }) async {
    try {
      final record = await _web5.dwn.records.update(
        web5.UpdateRecordOptions(
          recordId: recordId,
          data: jsonEncode(data),
        ),
      );

      _logger.info('Updated record: $recordId');
      return record;
    } catch (e) {
      _logger.severe('Failed to update record: $e');
      throw Web5Exception('Failed to update record: $e');
    }
  }

  /// Delete a record
  Future<void> deleteRecord(String recordId) async {
    try {
      await _web5.dwn.records.delete(
        web5.DeleteRecordOptions(
          recordId: recordId,
        ),
      );

      _logger.info('Deleted record: $recordId');
    } catch (e) {
      _logger.severe('Failed to delete record: $e');
      throw Web5Exception('Failed to delete record: $e');
    }
  }
}

/// Web5Service provides a unified interface for Web5 operations
/// with Read First Always principle enforcement and Bitcoin anchoring.
///
/// Part of AIP-001: Read First Always implementation.
class Web5ServiceWithReadFirst {
  final DidManager _didManager;
  final DwnManager _dwnManager;
  final ReadFirstDwnManager _readFirstManager;
  final BitcoinAnchoringService _anchoringService;
  
  /// Creates a new Web5Service with Read First Always enforcement
  Web5ServiceWithReadFirst(this._didManager, this._dwnManager, this._anchoringService) 
    : _readFirstManager = ReadFirstDwnManager(_didManager, _dwnManager);

  /// Get access to the ReadFirstMetrics for compliance tracking
  ReadFirstMetrics get metrics => _readFirstManager.metrics;
  
  /// Get the DID manager
  DidManager get didManager => _didManager;
  
  /// Create a new record in the DWN with Read First enforcement
  Future<RecordCreateResponse> createRecord(RecordCreateRequest request, {bool anchor = false}) async {
    final response = await _readFirstManager.createRecord(request);
    
    if (anchor && response.status.code == 200) {
      await _anchoringService.anchorRecord(response.recordId);
    }
    
    return response;
  }
  
  /// Query for records in the DWN with Read First tracking
  Future<RecordQueryResponse> queryRecords(RecordQueryRequest request) async {
    return await _readFirstManager.queryRecords(request);
  }
  
  /// Read a specific record by ID with Read First tracking
  Future<RecordReadResponse> readRecord(RecordReadRequest request) async {
    return await _readFirstManager.readRecord(request);
  }
  
  /// Update a record in the DWN with Read First enforcement
  Future<RecordUpdateResponse> updateRecord(RecordUpdateRequest request, {bool anchor = false}) async {
    final response = await _readFirstManager.updateRecord(request);
    
    if (anchor && response.status.code == 200) {
      await _anchoringService.anchorRecord(request.recordId);
    }
    
    return response;
  }
  
  /// Delete a record from the DWN with Read First enforcement
  Future<RecordDeleteResponse> deleteRecord(RecordDeleteRequest request, {bool anchor = false}) async {
    final response = await _readFirstManager.deleteRecord(request);
    
    if (anchor && response.status.code == 200) {
      await _anchoringService.anchorDeletion(request.recordId);
    }
    
    return response;
  }
  
  /// Get compliance report for Read First principle
  Map<String, dynamic> getComplianceReport() {
    return metrics.getDetailedMetrics();
  }
  
  /// Reset all Read First metrics
  void resetMetrics() {
    metrics.reset();
  }
  
  /// Check if a record is anchored to Bitcoin
  Future<AnchoringStatus> getAnchoringStatus(String recordId) async {
    return await _anchoringService.getAnchoringStatus(recordId);
  }
  
  /// Wait for a record to be confirmed on the Bitcoin blockchain
  Future<bool> waitForConfirmation(String recordId, {int minConfirmations = 1, Duration timeout = const Duration(minutes: 10)}) async {
    return await _anchoringService.waitForConfirmation(recordId, minConfirmations: minConfirmations, timeout: timeout);
  }
}

/// Custom exception for Web5 related errors
class Web5Exception implements Exception {
  final String message;
  Web5Exception(this.message);

  @override
  String toString() => 'Web5Exception: $message';
}
