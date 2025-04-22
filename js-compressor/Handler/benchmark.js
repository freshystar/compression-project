class Benchmark {
    static logMemoryUsage() {
        const memoryUsage = process.memoryUsage();
        console.log(`Memory Usage: ${JSON.stringify(memoryUsage)}`)
    }

    static measureExecutionTime(func) {
        const startTime = Date.now();
        func();
        const endTime = Date.now();
        return endTime - startTime;
    }
}

export default Benchmark;