#[bench]
fn inference_throughput(b: &mut Bencher) {
    let model = AnyaRuntime::load();
    b.iter(|| {
        model.process_batch(gen_test_inputs(1000)); // 1000 TPS target
    });
    
    assert!(b.elapsed() < Duration::from_millis(1000));
} 