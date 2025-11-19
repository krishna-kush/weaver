# Killcode Weaver

> ⚠️ **IMPORTANT: CURRENT IMPLEMENTATION STATUS**
> 
> **This is NOT the desired final implementation.** The current version achieves a **semi-desired behavior** sufficient to run Killcode, but falls short of the project's ultimate vision.
>
> ### Current State (Semi-Desired):
> - ✅ Links multiple binaries using a C loader stub
> - ✅ Basic binary protection and license enforcement
> - ⚠️ **Limitation:** Binaries remain **separable** - an inspector can extract individual components
> - ⚠️ **Limitation:** Uses external loader instead of true instruction-level weaving
>
> ### Desired Future State (True Weaving):
> 
> 1. **True Binary Weaving, Not Just Merging**
>    - This project is called "Weaver" not "Merger" for a reason
>    - Goal: Weave instructions from multiple binaries into a single, indivisible executable
>    - No external loaders or linking - pure instruction interleaving
>
> 2. **Inspector-Proof Single Binary**
>    - **Goal:** Create a monolithic binary where components are fundamentally inseparable
>    - Use jump instructions to flow between original binaries: `bin1_end → JMP → bin2_start → JMP → bin3_start`
>    - Interleave instructions at the assembly level
>    - **Challenge:** Stack and memory state management - binaries have hardcoded memory addresses and stack layouts that are extremely difficult to reconcile when merged
>    - No visible boundaries between original binaries in the instruction stream
>
> 3. **Instruction-Level Injection**
>    - **Goal:** Inject C/assembly instructions at arbitrary points in the binary
>    - Injection points: function boundaries, loop starts, conditional branches, syscalls
>    - Use cases: License checks before sensitive functions, telemetry at loop iterations, anti-debug at branch points
>    - **Challenge:** Requires deep binary analysis, control flow graph construction, and precise instruction patching without breaking relocations
>
> ### Why Not Implemented Yet?
> - **Stack/Memory State:** Each binary has hardcoded stack pointers, frame layouts, and memory assumptions
> - **Relocations:** Position-independent code assumptions break when merging instruction streams
> - **Control Flow:** Maintaining proper exception handling, signal handlers, and thread-local storage across merged binaries
> - **Debugging Symbols:** Must strip or carefully merge DWARF/PDB information
> - **Architecture Complexity:** x86-64, ARM, and other architectures have different calling conventions and instruction encoding
>
> The current loader-based approach is a pragmatic compromise that provides functional license enforcement while we work toward true instruction-level weaving.

---

High-performance binary weaving microservice for cross-platform executable merging with advanced health monitoring only for killcode support, not a core principle.

## Overview

Weaver is a stateless Rust microservice that merges license enforcement binaries (overload) with customer binaries across multiple architectures and operating systems. It handles platform detection, binary linking, loader stub generation, and optional execution testing.

## Key Features

### Multi-Architecture Support
- x86-64 (64-bit Intel/AMD)
- ARM64 (AArch64)
- ARM (32-bit)
- x86 (32-bit)
- Windows PE (MinGW)
- MIPS, PowerPC, RISC-V (detection)

### Multi-OS Support
- Linux (ELF) - Full support
- Windows (PE) - Merge support
- macOS (Mach-O) - Detection support

### Smart Binary Processing
- Automatic architecture detection using Goblin
- OS detection and validation
- Binary compatibility checking
- Automatic compiler selection
- Cross-compilation support

### Merge Capabilities
- Before Mode - Overload runs before base
- After Mode - Overload runs after base
- Loader stub generation
- Binary linking and packaging

### Execution Testing
- Native execution (x86-64)
- QEMU execution (ARM64, MIPS)
- Wine execution (Windows PE)

## Architecture

Receives binaries → Detects arch/OS → Validates compatibility → Generates loader → Compiles → Links → Tests → Stores → Publishes progress to Redis

## Merge Modes

### V1: Stop-on-Exit (Legacy)
Basic merging where overload runs first, then base binary.

**Endpoint:** `POST /merge/stop-on-exit`

### V2: Advanced Health Monitoring (Recommended)
Sophisticated merging with shared memory health monitoring:

**Features:**
- **Grace Period**: Network timeout tolerance (configurable seconds)
- **Sync Mode**: Wait for license verification before starting base binary
- **Async Mode**: Start base immediately, verify in background
- **Network Failure Threshold**: Kill base after N consecutive failures
- **Shared Memory IPC**: Real-time health status between processes
- **Fallback Kill**: Automatic termination if overload dies

**Endpoint:** `POST /merge/v2/stop-on-exit`

**Configuration:**
```json
{
  "base_data": "base64_encoded_binary",
  "overload_data": "base64_encoded_binary",
  "task_id": "unique_task_id",
  "grace_period": 300,           // seconds before timeout
  "sync_mode": true,             // wait for verification
  "network_failure_kill_count": 5  // max consecutive failures
}
```

## API Endpoints

### Core Endpoints
- `GET /health` - Service health check
- `POST /merge` - Basic merge (legacy)
- `POST /merge/stop-on-exit` - V1 merge with stop-on-exit
- `POST /merge/v2/stop-on-exit` - V2 merge with health monitoring
- `GET /download/{id}` - Download merged binary

### Response Format
```json
{
  "success": true,
  "binary_id": "uuid-v4",
  "download_url": "http://weaver:8080/download/{id}",
  "message": "Merge completed successfully"
}
```

## Environment Variables

```bash
# Server Configuration
WEAVER_HOST=0.0.0.0
WEAVER_PORT=8080
WEAVER_TEMP_DIR=/tmp/weaver

# Storage & Cleanup
WEAVER_EXPIRATION_HOURS=24      # Auto-cleanup after 24h
WEAVER_CLEANUP_INTERVAL=3600    # Cleanup check every hour
WEAVER_BINARY_TTL=3600          # In-memory cache TTL
WEAVER_MAX_SIZE=209715200       # Max upload: 200MB

# Integration
REDIS_URL=redis://redis:6379
MAIN_SERVER_URL=http://server:8080

# Testing (Development Only)
WEAVER_ENABLE_CROSS_HOST_TESTING=false  # Enable QEMU/Wine testing
```

## Tech Stack

- **Language:** Rust 1.91+
- **Framework:** Actix-Web
- **Binary Parser:** Goblin (ELF/PE/Mach-O)
- **Progress:** Redis pub/sub
- **Runtime:** Tokio (async)
- **HTTP:** reqwest

## Binary Tools

- GCC - Native compilation
- x86_64-linux-gnu-gcc - x86-64 cross-compiler
- aarch64-linux-gnu-gcc - ARM64 cross-compiler
- arm-linux-gnueabi-gcc - ARM cross-compiler
- x86_64-w64-mingw32-gcc - Windows cross-compiler
- objcopy - Binary manipulation
- QEMU - Cross-architecture execution
- Wine - Windows execution on Linux

## How It Works

1. Receive base and overload binaries
2. Detect architecture and OS using Goblin
3. Validate compatibility (same arch/OS)
4. Select appropriate cross-compiler
5. Generate C loader stub
6. Convert binaries to object files
7. Compile loader stub
8. Link everything together
9. Test merged binary execution
10. Store with unique ID
11. Publish progress to Redis

## Testing

- Real binary compilation and merging
- Multi-architecture test coverage
- Cross-compilation verification
- QEMU execution testing
- Wine execution testing
- No fake binaries or mocks

## Architecture Detection

Weaver automatically detects binary format using Goblin parser:

**Linux (ELF):**
- Header magic: `0x7F 'E' 'L' 'F'`
- Machine type: x86-64, ARM64, ARM, x86, MIPS, PowerPC, RISC-V
- ABI validation

**Windows (PE):**
- Header magic: `'M' 'Z'`
- Machine type: x86-64, x86
- Subsystem validation

**macOS (Mach-O):**
- Header magic: `0xfeedface`, `0xfeedfacf`, etc.
- CPU type: x86-64, ARM64
- Detection only (merge not implemented)

## Merge Process

1. **Receive & Validate**
   - Accept base64-encoded binaries
   - Validate size limits (200MB default)
   - Extract to temp directory

2. **Binary Analysis**
   - Parse headers with Goblin
   - Detect OS and architecture
   - Validate compatibility (same platform)

3. **Loader Generation**
   - Generate C loader stub with configuration
   - Embed grace period, sync mode, failure threshold
   - Setup shared memory for health monitoring (V2)

4. **Object Conversion**
   - Convert binaries to ELF objects with `objcopy`
   - Create named sections (`_binary_base_start`, `_binary_overload_start`)
   - Preserve architecture alignment

5. **Compilation**
   - Compile loader stub with matching GCC toolchain
   - Link loader + base + overload into single binary
   - Apply optimization flags

6. **Testing (Optional)**
   - Native execution (x86-64 on x86-64 host)
   - QEMU execution (ARM on x86-64 host)
   - Wine execution (Windows PE on Linux)

7. **Storage & Response**
   - Store in temp directory with UUID
   - Cache metadata in memory (HashMap)
   - Return download URL
   - Publish progress to Redis

## Health Monitoring (V2)

### Shared Memory IPC

```c
typedef struct {
    time_t last_success;           // Last successful verification
    int consecutive_failures;       // Network failure counter
    int is_alive;                   // Heartbeat from overload
    int should_kill_base;           // Kill signal to base
    int parent_requests_kill;       // Kill signal from parent
} HealthStatus;
```

### Monitor Thread Logic

```
Every 5 seconds:
  ├─ Check grace period timeout
  │  └─ If exceeded → SIGTERM → SIGKILL base
  ├─ Check network failure threshold
  │  └─ If exceeded → Signal overload to kill parent
  └─ Check overload heartbeat
     └─ If dead → Terminate base
```

### Kill Cascade

1. **Network Timeout**: Monitor thread detects grace period exceeded
2. **Overload Signal**: Sets `parent_requests_kill = 1` in shared memory
3. **Overload Action**: Reads flag, executes kill method (shred/wipe)
4. **Fallback**: If overload fails, monitor thread kills directly

## Performance

**Build Times:**
- Single merge: 1-3 seconds
- Concurrent merges: Limited by CPU cores
- Caching: In-memory HashMap + disk storage

**Binary Sizes:**
- Overhead: ~50-200KB (loader stub)
- Final size: base + overload + loader

**Cleanup:**
- Auto-cleanup after 24 hours (configurable)
- Periodic sweep every hour
- Manual cleanup via filesystem

## Integration Example

```bash
# Merge with health monitoring
curl -X POST http://weaver:8080/merge/v2/stop-on-exit \
  -H "Content-Type: application/json" \
  -d '{
    "base_data": "'"$(base64 -w0 my_app)"'",
    "overload_data": "'"$(base64 -w0 overload)"'",
    "task_id": "task_123",
    "grace_period": 300,
    "sync_mode": true,
    "network_failure_kill_count": 5
  }'

# Response
{
  "success": true,
  "binary_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "download_url": "http://weaver:8080/download/a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}

# Download merged binary
curl -o merged_binary "http://weaver:8080/download/a1b2c3d4-e5f6-7890-abcd-ef1234567890"
chmod +x merged_binary
./merged_binary
```

## Security Considerations

**Stateless Design:**
- No persistent database
- Temp files auto-expire
- Restart-safe (in-memory cache rebuilds)

**Isolation:**
- Temp directory per merge operation
- UUID-based file naming
- Cleanup on error

**Validation:**
- Binary size limits
- Architecture compatibility checks
- Format validation (ELF/PE headers)

## Troubleshooting

### Merge Fails
```bash
# Check binary format
file base_binary overload_binary

# Verify architecture match
readelf -h base_binary    # For ELF
objdump -f base_binary    # For PE
```

### Download Returns 404
- Binary expired (24h default)
- Wrong UUID
- Weaver restarted (in-memory cache lost)

### Performance Issues
- Increase `WEAVER_MAX_SIZE` for large binaries
- Check disk space in temp directory
- Monitor concurrent merge load

## Development

### Building
```bash
cd weaver
cargo build --release
```

### Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_test

# With QEMU/Wine (requires setup)
WEAVER_ENABLE_CROSS_HOST_TESTING=true cargo test
```

### Adding New Architecture
1. Add toolchain to `Dockerfile.dev`
2. Update `CompilerConfig` in `src/core/binary/compiler.rs`
3. Add linker mapping
4. Test with real binary

## Status

✅ **Production Ready**

- Linux ELF: Full support (x86-64, ARM64, ARM, x86)
- Windows PE: Merge support (x86-64, x86)
- macOS Mach-O: Detection only
- Health monitoring: V2 tested and stable
- All architectures validated with real binaries
