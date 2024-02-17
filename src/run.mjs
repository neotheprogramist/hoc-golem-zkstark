import { GolemNetwork } from "@golem-sdk/golem-js";
import process from "process";

const hash = process.argv[2];
const value = process.argv[3];

(async () => {
  const golemClient = new GolemNetwork({
    package: "",
    yagna: {
      apiKey: "try_golem",
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
      imageTag: "neoprogram/golem-example:latest",
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
    await ctx
      .beginBatch()
      .uploadFile("resources/main.proof", "/app/resources/main.proof")
      .run(
        `cargo run --release --bin post ${hash} ${value} < resources/main.proof`
      )
      .end();
  });

  const result = await job.waitForResult();

  console.log(result);
})();
