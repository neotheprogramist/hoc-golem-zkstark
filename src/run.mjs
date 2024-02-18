import { GolemNetwork } from "@golem-sdk/golem-js";
import process from "process";

const hash = process.argv[2];
// const value = process.argv[3];

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
        "17502f76ba1866ba9935970a8f18142abc9f2db4ec49025f51b721abe1292ceb",
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
    let result = await ctx
      .beginBatch()
      // .uploadFile("resources/main.proof", "/app/resources/main.proof")
      .run(`ls -al`)
      // .downloadFile("ls-al.txt", "resources/ls-al.txt")
      .end();
    return result;
  });
})();
