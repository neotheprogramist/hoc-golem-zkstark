import { TaskExecutor } from "@golem-sdk/golem-js";

(async () => {
  const executor = await TaskExecutor.create({
    package: "a2e484209226bc4060e27243447c53b8a2fa6cdfad5ec5abb622e301",
    yagnaOptions: { apiKey: "hoc_golem_zkstark" },
  });
  try {
    const result = await executor.run(async (ctx) => (await ctx.run("ls -al")).stdout);
    console.log("Task result:", result);
  } catch (err) {
    console.error("An error occurred:", err);
  } finally {
    await executor.shutdown();
  }
})();
