import { createActor, canisterId } from 'declarations/open_hours_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';

export const anonymousBackend = createActor(canisterId);

export const connect = async () => {
	let authClient = await AuthClient.create();

	await new Promise((resolve) => {
		authClient.login({
			identityProvider:
				process.env.DFX_NETWORK === 'ic'
					? 'https://identity.ic0.app'
					: `http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/`,
			onSuccess: resolve
		});
	});
	const identity = authClient.getIdentity();

	console.log(identity.getPrincipal().toText());

	const agent = new HttpAgent({ identity });
	return createActor(canisterId, { agent });
};
