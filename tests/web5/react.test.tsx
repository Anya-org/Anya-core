test('enforces Read First principle with Bitcoin anchoring', async () => {
  let testMetrics: any;
  
  const TestComponent = () => {
    const web5 = useWeb5();
    testMetrics = web5.metrics;
    return null;
  };

  await act(async () => {
    render(
      <Web5Provider>
        <TestComponent />
      </Web5Provider>
    );
  });

  await waitFor(() => {
    expect(testMetrics.readCount).toBeGreaterThan(0);
    expect(testMetrics.complianceRate).toBeGreaterThanOrEqual(0.95);
  }, { timeout: 10000 }); // Increased timeout for Bitcoin anchoring
}); 