import { TaskExecutor } from "@golem-sdk/golem-js";

(async () => {
  const executor = await TaskExecutor.create({
    package: "83761d48159d90e7b2ffb2811fc44de0f52ef79d201d10fb651198c0",
    yagnaOptions: { apiKey: "479949e84e3e4458880646280e6344bc" },
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
