fn main() {
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        println!("unica {}", env!("CARGO_PKG_VERSION"));
        println!("stdio MCP orchestrator for Unica workflows");
        println!("Supported MCP methods: initialize, tools/list, tools/call");
        return;
    }

    unica_coder::interfaces::mcp::run_stdio();
}
