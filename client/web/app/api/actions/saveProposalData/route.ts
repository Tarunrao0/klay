import { connectToDatabase } from "@/app/(mongoDB)/connectDb";
import Proposal from "@/app/(mongoDB)/createrSchema";
import {
  createActionHeaders,
  NextActionPostRequest,
  ActionError,
  CompletedAction,
  ACTIONS_CORS_HEADERS,
} from "@solana/actions";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

export const GET = async (req: Request) => {
  return Response.json({ message: "Method not supported" } as ActionError, {
    status: 403,
    headers: ACTIONS_CORS_HEADERS,
  });
};

export const OPTIONS = GET;

export const POST = async (req: Request) => {
  await connectToDatabase();

  let body: NextActionPostRequest;
  try {
    body = await req.json();
  } catch (err) {
    return new Response(
      JSON.stringify({ error: "Invalid JSON in request body" }),
      { status: 400, headers: ACTIONS_CORS_HEADERS }
    );
  }

  const url = new URL(req.url);
  const name = url.searchParams.get("name") ?? "";
  const proposal = url.searchParams.get("proposal") ?? "";
  const userPubKey = url.searchParams.get("userPubKey") ?? "";
  const proposalId = crypto.randomUUID();

  if (!name || !proposal) {
    return new Response(
      JSON.stringify({ error: "Name and proposal are required." }),
      { status: 400, headers: ACTIONS_CORS_HEADERS }
    );
  }

  let signature: string;
  try {
    signature = body.signature;
    if (!signature) throw "Invalid signature";
  } catch (err) {
    return new Response(
      JSON.stringify({ error: 'Invalid "signature" provided' }),
      { status: 400, headers: ACTIONS_CORS_HEADERS }
    );
  }

  try {
    const status = await connection.getSignatureStatus(signature);
    if (!status || !status.value?.confirmationStatus) {
      throw "Unknown signature status";
    }

    if (
      status.value.confirmationStatus !== "confirmed" &&
      status.value.confirmationStatus !== "finalized"
    ) {
      return new Response(
        JSON.stringify({ error: "Signature not confirmed or finalized" }),
        { status: 400, headers: ACTIONS_CORS_HEADERS }
      );
    }

    const newProposal = new Proposal({
      proposalId,
      createrName: name,
      proposalDescription: proposal,
      createrPubKey: userPubKey,
    });

    await newProposal.save();

    const payload: CompletedAction = {
      type: "completed",
      title: "Proposal Created",
      icon: "https://picsum.photos/200",
      label: "Proposal Created",
      description: "Your proposal has been created.",
    };

    return new Response(JSON.stringify(payload), {
      headers: ACTIONS_CORS_HEADERS,
    });
  } catch (err) {
    console.error("Error in transaction or saving event:", err);
    return new Response(
      JSON.stringify({
        error: "Unable to confirm the provided signature or save the proposal.",
      }),
      { status: 500, headers: ACTIONS_CORS_HEADERS }
    );
  }
};
