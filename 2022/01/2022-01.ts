import fs from "fs";
import path from "path";

const inputFileName = "2022-01-input.txt";

const logic = (input: string) => {
    const arr = Array.from(input.split("\n"));
    console.log(arr)
};

const main = async () => {
    try {
        const input = await fs.promises.readFile(path.join(__dirname, inputFileName), "utf8");
        logic(input);
    } catch (err) {
        console.error(err);
        throw err;
    }
};

main();
