import {
  ActionPostResponse,
  createPostResponse,
  ActionGetResponse,
  ACTIONS_CORS_HEADERS,
} from "@solana/actions";
import { clusterApiUrl, Connection, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";


const MY_PUB_KEY = "6rSrLGuhPEpxGqmbZzV1ZttwtLXzGx8V2WEACXd4qnVH";
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

export const GET = async (req: Request) => {
  const payload: ActionGetResponse = {
    title: "Create Your Proposal",
    icon: 'https://picsum.photos/200',
    description: "Want to get some changes in our service? Put your questions and voting options, share on social media, and tag us. If a majority votes for your decision, we will execute it.",
    label: "Create Proposal",
    links: {
      actions: [
        {
          label: "Create",
          href: "/api/actions/create-proposal?name={name}&proposal={proposal}",
          parameters: [
            {
              type: "text",
              name: "name",
              label: "Enter your Name",
              required: true,
            },
            {
              type: "textarea",
              name: "proposal",
              label: "What changes you want to see?",
              required: true,
            },
          ],
        },
      ],
    },
  };

  return new Response(JSON.stringify(payload), {
    headers: ACTIONS_CORS_HEADERS,
  });
};

export const OPTIONS = GET;

export const POST = async (req: Request) => {
  try {
    // Parse URL parameters
    const url = new URL(req.url);
    const name = url.searchParams.get("name") ?? "";
    const proposal = url.searchParams.get("proposal") ?? "";

    // Validate name and proposal
    if (!name || !proposal) {
      return new Response(
        JSON.stringify({ error: "Name and proposal are required." }),
        {
          status: 400,
          headers: ACTIONS_CORS_HEADERS,
        }
      );
    }

    // Parse request body
    const body = await req.json();
    const userPubkey = new PublicKey(body.account);

    // Create a Solana transaction
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: userPubkey,
        toPubkey: new PublicKey(MY_PUB_KEY),
        lamports: 0,  // 0 lamports since it's just to confirm the transaction
      })
    );

    // Set fee payer and recent blockhash for the transaction
    transaction.feePayer = userPubkey;
    transaction.recentBlockhash = (await connection.getLatestBlockhash())
      .blockhash;

    // Create the ActionPostResponse payload
    const payload: ActionPostResponse = await createPostResponse({
      fields: {
        transaction,  // The transaction object
        message: "",  // Add any custom message here
        links: {
          next: {
            type: "post",
            href: `/api/actions/saveProposalData?name=${encodeURIComponent(name)}&proposal=${encodeURIComponent(proposal)}&userPubKey=${userPubkey.toString()}`,
          },
        },
      },
    });

    return new Response(JSON.stringify(payload), {
      headers: ACTIONS_CORS_HEADERS,
    });
  } catch (error) {
    console.error("Error processing POST request:", error);
    return new Response(
      JSON.stringify({ error: "Failed to process request" }),
      {
        status: 500,
        headers: ACTIONS_CORS_HEADERS,
      }
    );
  }
};
