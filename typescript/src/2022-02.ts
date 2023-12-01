import fs from "fs";
import path from "path";

const inputFileName = "2022-02-input.txt";

const logic = (input: string) => {
    const inputArr = [...input.split("\n")].slice(0, -1);

    console.log(
        "Part 1 | Total score based on strategy guide 1: ",
        strategy(inputArr, 1)
    );
    console.log(
        "Part 2 | Total score based on strategy guide 2: ",
        strategy(inputArr, 2)
    );
};

const strategy = (inputArr: string[], strat: 1 | 2) => {
    // win             (+6pt)
    // draw            (+3pt)
    // lose            ( 0pt)
    if (strat == 1) {
        // A opp(rock)
        // B opp(paper)
        // C opp(scissors)
        // X you(rock)     (+1pt)
        // Y you(paper)    (+2pt)
        // Z you(scissors) (+3pt)
        return inputArr.reduce((acc: number, cur: string) => {
            const [opponent, yours] = cur.split(" ");
            acc += yours === "X" ? 1 : yours === "Y" ? 2 : 3;
            // Rock
            if (opponent === "A") {
                acc += yours === "Y" ? 6 : yours === "X" ? 3 : 0; // [Paper, Rock, Scissors]
            // Paper
            } else if (opponent === "B") {
                acc += yours === "Z" ? 6 : yours === "Y" ? 3 : 0; // [Scissors, Paper, Rock]
            // Scissors
            } else {
                acc += yours === "X" ? 6 : yours === "Z" ? 3 : 0; // [Rock, Scissors, Paper]
            }
            return acc;
        }, 0);
    } else {
        // A opp(rock)
        // B opp(paper)
        // C opp(scissors)
        // X you(lose)     ( 0pt)
        // Y you(draw)     (+3pt)
        // Z you(win)      (+6pt)
        return inputArr.reduce((acc: number, cur: string) => {
            const [opponent, yours] = cur.split(" ");
            acc += yours === "X" ? 0 : yours === "Y" ? 3 : 6;
            // Rock
            if (opponent === "A") {
                acc += yours === "Y" ? 1 : yours === "X" ? 3 : 2; // [Rock, Scissors, Paper]
            } else if (opponent === "B") {
            // Paper
                acc += yours === "Y" ? 2 : yours === "X" ? 1 : 3; // [Paper, Rock, Scissors]
            // Scissors
            } else {
                acc += yours === "Y" ? 3 : yours === "X" ? 2 : 1; // [Scissors, Paper, Rock]
            }
            return acc;
        }, 0);
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
