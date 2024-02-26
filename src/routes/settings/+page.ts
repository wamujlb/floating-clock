import { initAppSettings } from '$lib/settings/store';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	return {
		data: await initAppSettings(),
	};
};
