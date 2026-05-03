import { json } from '@sveltejs/kit';
import { getBuildSnapshot, startBuild } from '$lib/server/builds';

export function GET() {
	return json(getBuildSnapshot());
}

export async function POST({ request }) {
	const body = await request.json().catch(() => null);
	const result = startBuild(body?.target);

	if (!result.ok) {
		return json({ error: result.error }, { status: result.status });
	}

	return json({ ok: true }, { status: result.status });
}
