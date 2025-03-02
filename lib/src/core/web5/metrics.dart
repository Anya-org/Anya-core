/// Web5 Read First metrics tracking
/// 
/// This class provides metrics tracking for the Read First Always principle,
/// including counting reads, writes, and violations.
/// 
/// Part of AIP-001: Read First Always implementation.

class ReadFirstMetrics {
  int _readCount = 0;
  int _writeCount = 0;
  int _violationCount = 0;
  final Map<String, int> _recordTypeReads = {};
  final Map<String, int> _recordTypeWrites = {};
  final Map<String, int> _recordTypeViolations = {};

  /// Gets the total number of read operations performed
  int get readCount => _readCount;

  /// Gets the total number of write operations performed
  int get writeCount => _writeCount;

  /// Gets the total number of read-first principle violations
  int get violationCount => _violationCount;

  /// Gets the compliance percentage (reads before writes)
  double get compliancePercentage {
    if (_writeCount == 0) return 100.0;
    return 100.0 * ((_writeCount - _violationCount) / _writeCount);
  }

  /// Log a read operation for a specific record type
  void logRead(String recordType) {
    _readCount++;
    _recordTypeReads[recordType] = (_recordTypeReads[recordType] ?? 0) + 1;
  }

  /// Log a write operation for a specific record type
  void logWrite(String recordType) {
    _writeCount++;
    _recordTypeWrites[recordType] = (_recordTypeWrites[recordType] ?? 0) + 1;
  }

  /// Log a violation of the read-first principle for a specific record type
  void logViolation(String recordType) {
    _violationCount++;
    _recordTypeViolations[recordType] = (_recordTypeViolations[recordType] ?? 0) + 1;
  }

  /// Get metrics for a specific record type
  Map<String, dynamic> getRecordTypeMetrics(String recordType) {
    return {
      'reads': _recordTypeReads[recordType] ?? 0,
      'writes': _recordTypeWrites[recordType] ?? 0,
      'violations': _recordTypeViolations[recordType] ?? 0,
      'compliance': _getComplianceForRecordType(recordType),
    };
  }

  /// Get all metrics as a detailed report
  Map<String, dynamic> getDetailedMetrics() {
    final recordTypes = {..._recordTypeReads.keys, ..._recordTypeWrites.keys, ..._recordTypeViolations.keys};
    
    final recordTypeMetrics = <String, Map<String, dynamic>>{};
    for (final type in recordTypes) {
      recordTypeMetrics[type] = getRecordTypeMetrics(type);
    }
    
    return {
      'summary': {
        'readCount': _readCount,
        'writeCount': _writeCount,
        'violationCount': _violationCount,
        'compliancePercentage': compliancePercentage,
      },
      'recordTypes': recordTypeMetrics,
    };
  }

  /// Reset all metrics
  void reset() {
    _readCount = 0;
    _writeCount = 0;
    _violationCount = 0;
    _recordTypeReads.clear();
    _recordTypeWrites.clear();
    _recordTypeViolations.clear();
  }

  // Private helper method to calculate compliance for a specific record type
  double _getComplianceForRecordType(String recordType) {
    final writes = _recordTypeWrites[recordType] ?? 0;
    final violations = _recordTypeViolations[recordType] ?? 0;
    
    if (writes == 0) return 100.0;
    return 100.0 * ((writes - violations) / writes);
  }
}
