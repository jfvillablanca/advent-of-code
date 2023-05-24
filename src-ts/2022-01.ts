import fs from "fs";
import path from "path";

const inputFileName = "2022-01-input.txt";

const logic = (input: string) => {
    const inputArr = [...input.split("\n")].map((x) =>
        x === "" ? "" : Number(x)
    );

    const outputArr = inputArr.reduce((acc: number[], cur: number | "") => {
        if (cur === "") {
            acc.push(0);
        } else {
            const lastIndex = acc.length - 1;
            acc[lastIndex] += typeof cur === "number" ? cur : 0;
        }
        return acc;
    }, []);

    console.log("Part 1 | Highest total num: ", outputArr.sort((a, b) => b - a)[0]);
    console.log(
        "Part 2 | Total sum of top 3: ",
        outputArr
            .sort((a, b) => b - a)
            .slice(0, 3)
            .reduce((acc, cur) => acc + cur, 0)
    );
};

const main = async () => {
    try {
        const input = await fs.promises.readFile(
            path.join(__dirname, "../shared", inputFileName),
            "utf8"
        );
        logic(input);
    } catch (err) {
        console.error(err);
        throw err;
    }
};

main();
