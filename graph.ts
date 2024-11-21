const start = 0;
const end = 132;
const step = 3;
const width = 220;

const res = await fetch("https://tbjtest.yourbcabus.com/graphql", {
    method: "POST",
    headers: {
        accept: "application/json",
        "accept-encoding": "gzip, deflate",
        "content-type": "application/json",
    },
    body: JSON.stringify({
        query: `
        query Metrics {
          getMetrics {
            buckets(start:${start}, end:${end}, step:${step}) {
              values
              graph(maxWidth:${width})
            }
          }
        }
        `,
    }),
});

const json = await res.json();


console.log("_".repeat(width + 4));
console.log(json.data.getMetrics.buckets.graph.split("\n").map(s => `| ${s.padEnd(width, " ")} |`).join("\n"));
console.log("~".repeat(width + 4));

