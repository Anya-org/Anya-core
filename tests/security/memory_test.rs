#![feature(edition2021)]
#[test]
fn test_memory_isolation() {
    #[cfg(target_os = "linux")]
    {
        let result = check_memory_isolation()
            .expect("Memory isolation check failed");
        assert!(result, "System must have kernel.yama.ptrace_scope=1");
    }
} 