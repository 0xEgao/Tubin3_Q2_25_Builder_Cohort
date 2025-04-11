import bs58 from "bs58";
function base58ToByteArray(base58Key: string): Uint8Array {
    return bs58.decode(base58Key);
}
const base58PrivateKey = "5MNc9kbAo725MsrreFHVuaNwoG5DmtcyBj75FxiVcQ1b1w27qvHgj9wdzVBWdWFFtLN7aT9cTH5G3fJc3FjpHhcB";
const byteArray = base58ToByteArray(base58PrivateKey);

console.log("Byte Array:", byteArray);
//pasted to  Turbin3-wallet.json
