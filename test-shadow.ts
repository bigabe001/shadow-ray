// Import the actual Client from the source
import { Client } from './shadow-ray-client/src/index.ts';

async function checkContract() {
  console.log("--- Shadow Ray Bridge Check ---");

  try {
    const client = new Client({
      networkPassphrase: 'Test SDF Network ; September 2015',
      rpcUrl: 'https://soroban-testnet.stellar.org',
      // Your new VERIFIED contract ID:
      contractId: 'CDXCMTYSHFUL32A63WXHNAYHD3C2IRNQHVB72BTO2S2NQUICMMMR35U7',
    });

    // In some versions of the generator, the methods are on 'client' directly
    // in others, they are under 'client.tx'. Let's find where they are:
    const target = (client as any).tx || client;
    
    console.log("Looking for methods...");
    const methods = Object.keys(target);
    
    // Filter out internal SDK methods to see your contract functions
    const contractMethods = methods.filter(m => !['options', 'networkPassphrase', 'rpcUrl'].includes(m));
    
    console.log("Contract methods found:", contractMethods);

    if (contractMethods.includes('commit_shadow')) {
      console.log("✅ SUCCESS: The frontend is now linked to 'commit_shadow'!");
    } else {
      console.log("⚠️ Method not found. You might need to re-run 'stellar contract bindings'.");
    }
  } catch (e) {
    console.error("Initialization error:", e);
  }
}

checkContract();