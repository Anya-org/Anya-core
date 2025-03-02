/// Read First DWN Manager for Web5
///
/// This class implements the Read First Always principle for Web5 DWN operations,
/// ensuring that every write operation is preceded by a read operation.
///
/// Part of AIP-001: Read First Always implementation.

import 'dart:async';
import 'package:web5_dart/web5_dart.dart';
import 'metrics.dart';

/// Manager that enforces the Read First Always principle for DWN operations
class ReadFirstDwnManager {
  final DidManager _didManager;
  final DwnManager _dwnManager;
  final ReadFirstMetrics _metrics;
  
  // Track which records have been read before writing
  final Map<String, bool> _readRecords = {};

  /// Creates a new ReadFirstDwnManager instance
  ReadFirstDwnManager(this._didManager, this._dwnManager) 
    : _metrics = ReadFirstMetrics();

  /// Get metrics tracking object
  ReadFirstMetrics get metrics => _metrics;

  /// Create a new record in the DWN
  /// Enforces Read First by attempting to read first (which will fail gracefully for new records)
  Future<RecordCreateResponse> createRecord(RecordCreateRequest request) async {
    // For new records, we attempt to query for similar records first
    await _queryBeforeWrite(request.recordId, request.dataFormat);
    
    // Mark this record as having been properly processed
    _readRecords[request.recordId] = true;
    
    // Log the write operation
    _metrics.logWrite(request.dataFormat);
    
    // Proceed with the creation
    return await _dwnManager.createRecord(request);
  }

  /// Query for records in the DWN
  Future<RecordQueryResponse> queryRecords(RecordQueryRequest request) async {
    // Log the read operation
    _metrics.logRead(request.filter.dataFormat ?? 'unknown');
    
    // Perform the query
    final response = await _dwnManager.queryRecords(request);
    
    // Mark all returned records as having been read
    for (final record in response.records) {
      _readRecords[record.recordId] = true;
    }
    
    return response;
  }

  /// Read a specific record by ID
  Future<RecordReadResponse> readRecord(RecordReadRequest request) async {
    // Log the read operation
    _metrics.logRead('record');
    
    // Perform the read
    final response = await _dwnManager.readRecord(request);
    
    // Mark this record as having been read
    _readRecords[request.recordId] = true;
    
    return response;
  }

  /// Update a record in the DWN
  /// Enforces Read First by requiring a prior read of the record
  Future<RecordUpdateResponse> updateRecord(RecordUpdateRequest request) async {
    // Check if we've read this record before
    if (!(_readRecords[request.recordId] ?? false)) {
      // We haven't read this record yet, so read it first
      await _enforceReadFirst(request.recordId);
    }
    
    // Log the write operation
    _metrics.logWrite('record');
    
    // Proceed with the update
    return await _dwnManager.updateRecord(request);
  }

  /// Delete a record from the DWN
  /// Enforces Read First by requiring a prior read of the record
  Future<RecordDeleteResponse> deleteRecord(RecordDeleteRequest request) async {
    // Check if we've read this record before
    if (!(_readRecords[request.recordId] ?? false)) {
      // We haven't read this record yet, so read it first
      await _enforceReadFirst(request.recordId);
    }
    
    // Log the write operation
    _metrics.logWrite('record');
    
    // Proceed with the deletion
    return await _dwnManager.deleteRecord(request);
  }

  // Private helper methods
  
  // Enforces Read First by reading the record before proceeding
  Future<void> _enforceReadFirst(String recordId) async {
    try {
      // Attempt to read the record first
      final readRequest = RecordReadRequest(recordId: recordId);
      await readRecord(readRequest);
    } catch (e) {
      // Log a violation if we can't read the record
      _metrics.logViolation('record');
      
      // We still mark it as read to avoid repeated violations
      _readRecords[recordId] = true;
    }
  }
  
  // Query for similar records before writing
  Future<void> _queryBeforeWrite(String recordId, String dataFormat) async {
    try {
      // Query for records with the same data format
      final queryRequest = RecordQueryRequest(
        filter: RecordFilter(
          dataFormat: dataFormat,
        ),
      );
      await queryRecords(queryRequest);
    } catch (e) {
      // If query fails, we still want to allow the creation
      // but we'll log it as a violation
      _metrics.logViolation(dataFormat);
    }
  }
}
