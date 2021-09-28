function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

function getNew(pieces) {
    // should pieces be "scrambled" each time?
    var newPieces = [];
    for (let i = 0; i < 8; i++) {
        let randIndex = getRandomInt(8-i);
        let piece = pieces[randIndex];
        newPieces.push(piece);
        piecesFore = pieces.slice(0, randIndex);
        piecesAft = pieces.slice(randIndex+1, pieces.length);
        pieces = [...piecesFore, ...piecesAft];
    }
    return newPieces;
}

function generateBackboard() {
    const pieces = ['k', 'q', 'r', 'r', 'b', 'b', 'n', 'n'];
    var pieces2 = getNew(pieces);
    var tries = 1;
    while (!validatePieces(pieces2)) {
        pieces2 = getNew(pieces);
        tries++;
    }
    console.log("Attempts made: " + tries);
    return pieces2;
}

function validatePieces(pieces) {
    var bishopFirstIs = 'null';
    var rookIn = false;
    var rookInKingIn = false;
    var rookOut = false;
    var i = 0;
    for (let piece of pieces) {
        switch (piece) {
            case 'b':
                // console.log(i);
                // console.log(bishopFirstIs);
                if (bishopFirstIs == 'null') {
                    if (i % 2 == 0) {
                        bishopFirstIs = 'even';
                    } else {
                        bishopFirstIs = 'odd';
                    }
                    // console.log(bishopFirstIs);
                } else {
                    if (i % 2 == 0) { // index is even
                        // console.log("should be odd, but is " + bishopFirstIs);
                        if (bishopFirstIs == 'even') {
                            return false;
                        }
                    } else {
                        // console.log("should be even, but is " + bishopFirstIs);
                        if (bishopFirstIs == 'odd') {
                            return false;
                        }
                    }
                } 
                break;
            case 'r':
                if (rookIn) {
                    if (!rookInKingIn) {
                        return false;
                    } else {
                        rookOut = true;
                    }
                } else {
                    rookIn = true;
                }
                break;
            case 'k':
                if (rookIn) {
                    rookInKingIn = true;
                }
                break;
            case 'x':
                return false;
        }
        i++;
    }
    if (rookOut) {
        return true;
    } else return false;
}