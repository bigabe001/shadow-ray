import { Client } from './shadow-ray-client/src/index.ts';
import { isAllowed, setAllowed, getUserInfo, signTransaction } from "@stellar/freighter-api";

const client = new Client({
  networkPassphrase: 'Test SDF Network ; September 2015',
  rpcUrl: 'https://soroban-testnet.stellar.org',
  contractId: 'CDUPGEQTHGWYJBJH7V75XGFQ6ZYVBIF7DIQTYGAXFUAM3WXWGV7WYISR',
});

export default function ShadowPortal() {
  const handleCommit = async () => {
    // 1. Check if Freighter is connected
    if (!await isAllowed()) {
      await setAllowed();
    }

    // 2. Get user's public key
    const { publicKey } = await getUserInfo();
    console.log("Connected as:", publicKey);

    // 3. Prepare the 'commit_shadow' transaction
    // Note: We use 12345 as a dummy session_id for this test
    const tx = await client.commit_shadow({
      session_id: 12345,
    });

    // 4. Sign and Send
    const result = await tx.signAndSend();
    console.log("Transaction Result:", result);
    alert("Shadow Committed to the Blockchain!");
  };

  return (
    <div style={{ padding: '20px', textAlign: 'center' }}>
      <h1>Shadow Ray Portal</h1>
      <button onClick={handleCommit} style={{ padding: '10px 20px', cursor: 'pointer' }}>
        Commit Shadow Move
      </button>
    </div>
  );
}