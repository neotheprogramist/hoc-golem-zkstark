import { GolemNetwork } from "@golem-sdk/golem-js";
import process from "process";

const hash = process.argv[2];
const value = process.argv[3];

(async () => {
  const golemClient = new GolemNetwork({
    yagna: {
      apiKey: "hoc_golem_zkstark",
    },
  });

  await golemClient
    .init()
    .then(() => {
      console.log("Connected to the Golem Network!");
    })
    .catch((error) => {
      console.error("Failed to connect to the Golem Network:", error);
      process.exit(1);
    });

  const job = golemClient.createJob({
    package: {
      imageHash:
        "815007a709fad494bda705041383870416d19e4230f701907898f9d9885e0413",
    },
  });

  job.events.on("created", () => {
    console.log("Job created");
  });
  job.events.on("started", () => {
    console.log("Job started");
  });
  job.events.on("error", () => {
    console.log("Job failed", job.error);
  });
  job.events.on("success", () => {
    console.log("Job succeeded", job.results);
  });

  job.startWork(async (ctx) => {
    let result = await ctx.beginBatch().run(`get ${hash}`).end();
    return result;
  });

  await job.waitForResult();

  job.startWork(async (ctx) => {
    let result = await ctx
      .beginBatch()
      .uploadFile("resources/main.proof", "/main.proof")
      .run(`post ${hash} ${value} < /main.proof`)
      .end();
    return result;
  });

  await job.waitForResult();

  job.startWork(async (ctx) => {
    let result = await ctx.beginBatch().run(`get ${hash}`).end();
    return result;
  });

  await job.waitForResult();
})();
