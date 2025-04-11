import wallet from "../wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const image=await readFile("./cluster1/asset/zeke.gif");
        //2. Convert image to generic file.
        const genericFile = createGenericFile(image,"zeke.gif",{contentType:"image/gif"});
        //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]); 
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
//https://devnet.irys.xyz/JjqLE4vWhtnxoVboKUBVdEh75qJKzALbx2DpxpkFLQg
//https://devnet.irys.xyz/FPy42LCntVNNYq3gpeVrLAfaTePv5NtodFhzkeoyojru
//https://devnet.irys.xyz/FhxCJK5DJ5c7tqxKWxPqeWCMnnWQsw28QG6W4MirBexF