import fs from "fs";
import path from "path";

const inputFileName = "2022-01-input.txt";

const logic = (input: string) => {
    const elfArr = [];
    const inputArr = [...input.split("\n")].map((x) =>
        x === "" ? "X" : Number(x)
    );

    let tempArr: number[] = [];
    for (const num of inputArr) {
        if (num === "X") {
            elfArr.push(tempArr.reduce((acc, cur) => acc + cur, 0));
            tempArr = [];
        } else {
            tempArr.push(num);
        }
    }
    console.log(elfArr.sort((a, b) => b - a)[0]);
    console.log(
        elfArr
            .sort((a, b) => b - a)
            .slice(0, 3)
            .reduce((acc, cur) => acc + cur, 0)
    );
};

const main = async () => {
    try {
        const input = await fs.promises.readFile(
            path.join(__dirname, inputFileName),
            "utf8"
        );
        logic(input);
    } catch (err) {
        console.error(err);
        throw err;
    }
};

main();
