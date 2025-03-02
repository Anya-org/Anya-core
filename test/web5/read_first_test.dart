import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:web5_dart/web5_dart.dart';
import 'package:anya_core/src/core/web5/metrics.dart';
import 'package:anya_core/src/core/web5/read_first_dwn.dart';
import 'package:anya_core/src/core/web5/web5_service.dart';

@GenerateMocks([DidManager, DwnManager, BitcoinAnchoringService])
import 'read_first_test.mocks.dart';

void main() {
  late MockDidManager mockDidManager;
  late MockDwnManager mockDwnManager;
  late MockBitcoinAnchoringService mockAnchoringService;
  late ReadFirstDwnManager readFirstManager;
  late Web5ServiceWithReadFirst web5Service;

  setUp(() {
    mockDidManager = MockDidManager();
    mockDwnManager = MockDwnManager();
    mockAnchoringService = MockBitcoinAnchoringService();
    readFirstManager = ReadFirstDwnManager(mockDidManager, mockDwnManager);
    web5Service = Web5ServiceWithReadFirst(mockDidManager, mockDwnManager, mockAnchoringService);
  });

  group('ReadFirstMetrics Tests', () {
    test('should track read operations', () {
      final metrics = ReadFirstMetrics();
      metrics.logRead('test');
      
      expect(metrics.readCount, 1);
      expect(metrics.getRecordTypeMetrics('test')['reads'], 1);
    });

    test('should track write operations', () {
      final metrics = ReadFirstMetrics();
      metrics.logWrite('test');
      
      expect(metrics.writeCount, 1);
      expect(metrics.getRecordTypeMetrics('test')['writes'], 1);
    });

    test('should track violations', () {
      final metrics = ReadFirstMetrics();
      metrics.logViolation('test');
      
      expect(metrics.violationCount, 1);
      expect(metrics.getRecordTypeMetrics('test')['violations'], 1);
    });

    test('should calculate compliance percentage correctly', () {
      final metrics = ReadFirstMetrics();
      
      // Initially 100% compliant (no writes)
      expect(metrics.compliancePercentage, 100.0);
      
      // Add one write, still 100% compliant
      metrics.logWrite('test');
      expect(metrics.compliancePercentage, 100.0);
      
      // Add one violation, now 50% compliant
      metrics.logViolation('test');
      expect(metrics.compliancePercentage, 50.0);
      
      // Add another write, now 66.7% compliant
      metrics.logWrite('test');
      expect(metrics.compliancePercentage, closeTo(66.7, 0.1));
    });

    test('should reset metrics correctly', () {
      final metrics = ReadFirstMetrics();
      metrics.logRead('test');
      metrics.logWrite('test');
      metrics.logViolation('test');
      
      metrics.reset();
      
      expect(metrics.readCount, 0);
      expect(metrics.writeCount, 0);
      expect(metrics.violationCount, 0);
      expect(metrics.getRecordTypeMetrics('test')['reads'], 0);
    });

    test('should generate detailed metrics report', () {
      final metrics = ReadFirstMetrics();
      metrics.logRead('typeA');
      metrics.logWrite('typeA');
      metrics.logRead('typeB');
      metrics.logViolation('typeB');
      
      final report = metrics.getDetailedMetrics();
      
      expect(report['summary']['readCount'], 2);
      expect(report['summary']['writeCount'], 1);
      expect(report['summary']['violationCount'], 1);
      expect(report['recordTypes']['typeA']['reads'], 1);
      expect(report['recordTypes']['typeB']['violations'], 1);
    });
  });

  group('ReadFirstDwnManager Tests', () {
    late RecordCreateRequest createRequest;
    late RecordCreateResponse createResponse;
    late RecordReadRequest readRequest;
    late RecordReadResponse readResponse;
    late RecordUpdateRequest updateRequest;
    late RecordUpdateResponse updateResponse;
    late RecordDeleteRequest deleteRequest;
    late RecordDeleteResponse deleteResponse;
    late RecordQueryRequest queryRequest;
    late RecordQueryResponse queryResponse;

    setUp(() {
      // Set up common test data
      createRequest = RecordCreateRequest(
        recordId: 'record123',
        dataFormat: 'application/json',
      );
      createResponse = RecordCreateResponse(
        recordId: 'record123',
        status: Status(code: 200, detail: 'OK'),
      );
      
      readRequest = RecordReadRequest(recordId: 'record123');
      readResponse = RecordReadResponse(
        record: Record(
          recordId: 'record123',
          data: Uint8List.fromList([1, 2, 3]),
          dataFormat: 'application/json',
        ),
        status: Status(code: 200, detail: 'OK'),
      );
      
      updateRequest = RecordUpdateRequest(
        recordId: 'record123',
        data: Uint8List.fromList([4, 5, 6]),
      );
      updateResponse = RecordUpdateResponse(
        status: Status(code: 200, detail: 'OK'),
      );
      
      deleteRequest = RecordDeleteRequest(recordId: 'record123');
      deleteResponse = RecordDeleteResponse(
        status: Status(code: 200, detail: 'OK'),
      );
      
      queryRequest = RecordQueryRequest(
        filter: RecordFilter(dataFormat: 'application/json'),
      );
      queryResponse = RecordQueryResponse(
        records: [
          Record(
            recordId: 'record123',
            data: Uint8List.fromList([1, 2, 3]),
            dataFormat: 'application/json',
          ),
        ],
        status: Status(code: 200, detail: 'OK'),
      );
      
      // Set up mock behaviors
      when(mockDwnManager.createRecord(any)).thenAnswer((_) async => createResponse);
      when(mockDwnManager.readRecord(any)).thenAnswer((_) async => readResponse);
      when(mockDwnManager.updateRecord(any)).thenAnswer((_) async => updateResponse);
      when(mockDwnManager.deleteRecord(any)).thenAnswer((_) async => deleteResponse);
      when(mockDwnManager.queryRecords(any)).thenAnswer((_) async => queryResponse);
    });

    test('create record should query first', () async {
      await readFirstManager.createRecord(createRequest);
      
      // Verify it performed a query before creating
      verify(mockDwnManager.queryRecords(any)).called(1);
      verify(mockDwnManager.createRecord(createRequest)).called(1);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      expect(readFirstManager.metrics.writeCount, 1);
      expect(readFirstManager.metrics.violationCount, 0);
    });

    test('read record should update read tracking', () async {
      await readFirstManager.readRecord(readRequest);
      
      verify(mockDwnManager.readRecord(readRequest)).called(1);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      expect(readFirstManager.metrics.writeCount, 0);
    });

    test('update record should read first if not already read', () async {
      await readFirstManager.updateRecord(updateRequest);
      
      // Verify it read the record first
      verify(mockDwnManager.readRecord(any)).called(1);
      verify(mockDwnManager.updateRecord(updateRequest)).called(1);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      expect(readFirstManager.metrics.writeCount, 1);
      expect(readFirstManager.metrics.violationCount, 0);
    });

    test('update record should not read again if already read', () async {
      // First read the record
      await readFirstManager.readRecord(readRequest);
      
      // Now update it
      await readFirstManager.updateRecord(updateRequest);
      
      // Verify it only read once (not before the update)
      verify(mockDwnManager.readRecord(any)).called(1);
      verify(mockDwnManager.updateRecord(updateRequest)).called(1);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      expect(readFirstManager.metrics.writeCount, 1);
    });

    test('delete record should read first if not already read', () async {
      await readFirstManager.deleteRecord(deleteRequest);
      
      // Verify it read the record first
      verify(mockDwnManager.readRecord(any)).called(1);
      verify(mockDwnManager.deleteRecord(deleteRequest)).called(1);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      expect(readFirstManager.metrics.writeCount, 1);
    });

    test('query records should update read tracking for all returned records', () async {
      await readFirstManager.queryRecords(queryRequest);
      
      verify(mockDwnManager.queryRecords(queryRequest)).called(1);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      
      // Now update without an explicit read
      await readFirstManager.updateRecord(updateRequest);
      
      // Should not need another read since query already marked it
      verify(mockDwnManager.readRecord(any)).called(0);
      
      // Check metrics
      expect(readFirstManager.metrics.readCount, 1);
      expect(readFirstManager.metrics.writeCount, 1);
      expect(readFirstManager.metrics.violationCount, 0);
    });
  });

  group('Web5Service with Read First Tests', () {
    test('should integrate with Bitcoin anchoring for create operations', () async {
      final createRequest = RecordCreateRequest(
        recordId: 'record123',
        dataFormat: 'application/json',
      );
      final createResponse = RecordCreateResponse(
        recordId: 'record123',
        status: Status(code: 200, detail: 'OK'),
      );
      
      when(mockDwnManager.createRecord(any)).thenAnswer((_) async => createResponse);
      when(mockDwnManager.queryRecords(any)).thenAnswer((_) async => RecordQueryResponse(
        records: [],
        status: Status(code: 200, detail: 'OK'),
      ));
      
      await web5Service.createRecord(createRequest, anchor: true);
      
      // Verify it anchored the record
      verify(mockAnchoringService.anchorRecord('record123')).called(1);
    });

    test('should integrate with Bitcoin anchoring for update operations', () async {
      final readRequest = RecordReadRequest(recordId: 'record123');
      final readResponse = RecordReadResponse(
        record: Record(recordId: 'record123', data: Uint8List.fromList([1, 2, 3]), dataFormat: 'application/json'),
        status: Status(code: 200, detail: 'OK'),
      );
      
      final updateRequest = RecordUpdateRequest(
        recordId: 'record123',
        data: Uint8List.fromList([4, 5, 6]),
      );
      final updateResponse = RecordUpdateResponse(
        status: Status(code: 200, detail: 'OK'),
      );
      
      when(mockDwnManager.readRecord(any)).thenAnswer((_) async => readResponse);
      when(mockDwnManager.updateRecord(any)).thenAnswer((_) async => updateResponse);
      
      // First read to establish tracking
      await web5Service.readRecord(readRequest);
      
      // Now update with anchoring
      await web5Service.updateRecord(updateRequest, anchor: true);
      
      // Verify it anchored the update
      verify(mockAnchoringService.anchorRecord('record123')).called(1);
    });

    test('should provide compliance reporting', () async {
      // Perform some operations
      final readRequest = RecordReadRequest(recordId: 'record123');
      final readResponse = RecordReadResponse(
        record: Record(recordId: 'record123', data: Uint8List.fromList([1, 2, 3]), dataFormat: 'application/json'),
        status: Status(code: 200, detail: 'OK'),
      );
      
      when(mockDwnManager.readRecord(any)).thenAnswer((_) async => readResponse);
      
      await web5Service.readRecord(readRequest);
      
      // Get compliance report
      final report = web5Service.getComplianceReport();
      
      expect(report['summary']['readCount'], 1);
      expect(report['summary']['compliancePercentage'], 100.0);
    });
  });
}
