import fs from "fs";
import path from "path";

const inputFileName = "2022-03-input.txt";

const logic = (input: string) => {
    const inputArr = [...input.split("\n")].slice(0, -1);

    const priorityByCompartment = inputArr.reduce((acc, currentLine) => {
        const [compartmentOne, compartmentTwo] = splitLine(currentLine);
        const commonItem = findCommonElements(
            compartmentOne,
            compartmentTwo
        )[0];
        acc += mapCharToValue(commonItem);
        return acc;
    }, 0);

    console.log(
        "Part 1 | Sum of priorities between rucksack compartments: ",
        priorityByCompartment
    );
};

const splitLine = (line: string): [string[], string[]] => {
    const halfIndex = line.length / 2;
    return [[...line.slice(0, halfIndex)], [...line.slice(halfIndex)]];
};

const findCommonElements = (a: string[], b: string[]) => {
    const set = new Set(a);
    const commonElements = [];

    for (let i = 0; i < b.length; i++) {
        if (set.has(b[i])) {
            commonElements.push(b[i]);
        }
    }
    return [...new Set(commonElements)];
};

const mapCharToValue = (c: string): number => {
    const code = c.charCodeAt(0);
    if (code >= 97 && code <= 122) {
        return code - 96;
    } else if (code >= 65 && code <= 90) {
        return code - 38;
    } else {
        return 0; // or handle invalid characters accordingly
    }
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
