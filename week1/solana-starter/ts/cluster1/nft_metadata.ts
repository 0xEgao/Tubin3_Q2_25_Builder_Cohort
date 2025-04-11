import wallet from "../wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        //https://developers.metaplex.com/token-metadata/guides/javascript/create-an-nft#initializing
        const image = "https://devnet.irys.xyz/FPy42LCntVNNYq3gpeVrLAfaTePv5NtodFhzkeoyojru";
        const animation="https://devnet.irys.xyz/8BNFoPhNYvDWFSDm9gbwVqzC8i9wYgB2HK3GUCgDLbCh";
         const metadata = {
             name: "ZEKE",
             symbol: "BEAST",
             description: "testing animation",
             image: animation,             
             attributes: [  
                 { trait_type: 'animation', value: 'true' },
             ],
             properties: {
                 files: [
                     {
                         type: "image/gif",
                         uri: animation
                     },
                 ],
             },
         };
        const myUri = JSON.stringify(metadata);
        //2. Convert image to generic file.
        const genericFile = createGenericFile(myUri, "metadata.json", { contentType: "application/json" });
        //3. Upload image
        const [Uri] = await umi.uploader.upload([genericFile]);
         console.log("Your metadata URI: ", Uri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
//https://devnet.irys.xyz/3gCGCDAWLwnajRGZbkR3Q8CQ58ypRaJkpyf8rwSV6Yv7
//https://devnet.irys.xyz/BH1XWbWXCzAAzTWc7tB6E1h1ZVtqPP9eq6MZ6QF6Q2v8
// 3EagX2WF2FP7vchYTZaYFbbVTrr4nn2z8ktq61bN87Fh
//3GFLBqPkgm9NXLr2Ax1wQSk5bMvNFqY5w4fYYuyEoHGp