/** @fileoverview A gRPC client for fetching CPU usage data. */

const grpc = require("@grpc/grpc-js");

const msgs = require("./cpu_usage_pb.js");

const svcs = require("./cpu_usage_grpc_pb.js");

/** @type {string} */
const addr = "127.0.0.1:7151";

/** @type {svcs.CpuUsageClient} */
const cli = new svcs.CpuUsageClient(
  addr,
  grpc.credentials.createInsecure(),
);

/** @type {msgs.CpuUsageRequest} */
const req = new msgs.CpuUsageRequest();
req.setWaitDurationMs(1000);

/**
 * Callback function for getCpuUsage.
 * @param {Error | null} err - The error object if an error occurred, otherwise null.
 * @param {msgs.CpuUsageResponse} res - The CPU usage response.
 */
cli.getCpuUsage(req, (err, res) => {
  if (err) {
    console.error("Error fetching CPU usage:", err);
    return;
  }

  const list = res.getStatsList();

  for (const item of list) {
    /** @type number */
    const cpuid = item.getCpuid();

    /** @type number */
    const usage = item.getUsage();

    console.info({ cpuid, usage });
  }
});
