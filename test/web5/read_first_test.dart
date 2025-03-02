import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:web5_dart/web5_dart.dart' as web5;
import 'package:anya_core/src/core/web5/read_first_dwn.dart';
import 'package:anya_core/src/core/web5/metrics.dart';
import 'package:anya_core/src/core/web5/web5_service.dart';

// Generate the mocks
@GenerateMocks([web5.Web5Client, web5.DWNRecordsApi])
import 'read_first_test.mocks.dart';

void main() {
  late MockWeb5Client mockWeb5Client;
  late ReadFirstDwnManager readFirstDwnManager;
  late MockDWNRecordsApi mockDwnRecordsApi;

  setUp(() {
    mockWeb5Client = MockWeb5Client();
    mockDwnRecordsApi = MockDWNRecordsApi();
    
    // Setup the mock Web5Client to return our mock DWNRecordsApi
    when(mockWeb5Client.dwn).thenReturn(
      web5.DWN(records: mockDwnRecordsApi),
    );
    
    readFirstDwnManager = ReadFirstDwnManager(mockWeb5Client);
  });

  group('ReadFirstDwnManager', () {
    test('creates record after querying similar records', () async {
      // Mock data
      final options = web5.CreateRecordOptions(
        data: '{"test": "data"}',
        dataFormat: 'application/json',
        schema: 'test-schema',
      );
      
      final mockRecord = web5.Record(
        id: 'test-id',
        data: '{"test": "data"}',
      );
      
      // Mock the query behavior
      when(mockDwnRecordsApi.query(any)).thenAnswer((_) async => []);
      
      // Mock the create behavior
      when(mockDwnRecordsApi.create(any)).thenAnswer((_) async => mockRecord);
      
      // Call the method
      final result = await readFirstDwnManager.createRecord(options);
      
      // Verify that query was called before create
      verifyInOrder([
        mockDwnRecordsApi.query(any),
        mockDwnRecordsApi.create(any),
      ]);
      
      // Check the result
      expect(result.id, equals('test-id'));
    });
    
    test('updates record after reading existing record', () async {
      // Mock data
      final recordId = 'test-id';
      final options = web5.UpdateRecordOptions(
        data: '{"updated": "data"}',
        dataFormat: 'application/json',
      );
      
      final existingRecord = web5.Record(
        id: recordId,
        data: '{"test": "data"}',
      );
      
      final updatedRecord = web5.Record(
        id: recordId,
        data: '{"updated": "data"}',
      );
      
      // Mock the read behavior
      when(mockDwnRecordsApi.read(recordId)).thenAnswer((_) async => existingRecord);
      
      // Mock the update behavior
      when(mockDwnRecordsApi.update(recordId, any)).thenAnswer((_) async => updatedRecord);
      
      // Call the method
      final result = await readFirstDwnManager.updateRecord(recordId, options);
      
      // Verify that read was called before update
      verifyInOrder([
        mockDwnRecordsApi.read(recordId),
        mockDwnRecordsApi.update(recordId, any),
      ]);
      
      // Check the result
      expect(result.id, equals(recordId));
      expect(result.data, equals('{"updated": "data"}'));
    });
    
    test('throws exception when trying to update non-existent record', () async {
      // Mock data
      final recordId = 'non-existent-id';
      final options = web5.UpdateRecordOptions(
        data: '{"updated": "data"}',
        dataFormat: 'application/json',
      );
      
      // Mock the read behavior to return null (record not found)
      when(mockDwnRecordsApi.read(recordId)).thenAnswer((_) async => null);
      
      // Expect an exception
      expect(
        () => readFirstDwnManager.updateRecord(recordId, options),
        throwsException,
      );
      
      // Verify that read was called but update was not
      verify(mockDwnRecordsApi.read(recordId)).called(1);
      verifyNever(mockDwnRecordsApi.update(recordId, any));
    });
    
    test('deletes record after reading existing record', () async {
      // Mock data
      final recordId = 'test-id';
      
      final existingRecord = web5.Record(
        id: recordId,
        data: '{"test": "data"}',
      );
      
      // Mock the read behavior
      when(mockDwnRecordsApi.read(recordId)).thenAnswer((_) async => existingRecord);
      
      // Mock the delete behavior
      when(mockDwnRecordsApi.delete(recordId)).thenAnswer((_) async => true);
      
      // Call the method
      final result = await readFirstDwnManager.deleteRecord(recordId);
      
      // Verify that read was called before delete
      verifyInOrder([
        mockDwnRecordsApi.read(recordId),
        mockDwnRecordsApi.delete(recordId),
      ]);
      
      // Check the result
      expect(result, isTrue);
    });
    
    test('metrics track reads and writes correctly', () async {
      // Mock data
      final recordId = 'test-id';
      
      final existingRecord = web5.Record(
        id: recordId,
        data: '{"test": "data"}',
      );
      
      // Mock the read behavior
      when(mockDwnRecordsApi.read(recordId)).thenAnswer((_) async => existingRecord);
      
      // Mock the update behavior
      when(mockDwnRecordsApi.update(recordId, any)).thenAnswer(
        (_) async => web5.Record(id: recordId, data: '{"updated": "data"}')
      );
      
      // Reset metrics
      ReadFirstMetrics().reset();
      
      // Perform operations
      await readFirstDwnManager.readRecord(recordId); // 1 read
      await readFirstDwnManager.updateRecord(
        recordId, 
        web5.UpdateRecordOptions(data: '{"updated": "data"}', dataFormat: 'application/json')
      ); // 1 read + 1 write
      
      // Get metrics
      final metrics = readFirstDwnManager.getMetrics();
      
      // Check metrics
      expect(metrics['read_count'], equals(2));
      expect(metrics['write_count'], equals(1));
      expect(metrics['violation_count'], equals(0));
      expect(metrics['compliance_rate'], equals(100.0));
    });
  });
}
