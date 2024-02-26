import { Store } from 'tauri-plugin-store-api';
import { DEFAULT_SETTINGS, deserializeSettings, parseSettingsPayloadJson } from './utils';

export const appStore = new Store('.settings.dat');

export const initAppSettings = async (): Promise<App.Settings> => {
	const res = await appStore.get<string>('settings');

	try {
		return res ? deserializeSettings(parseSettingsPayloadJson(res)) : DEFAULT_SETTINGS;
	} catch (error) {
		console.error('Error parsing settings, returned default settings instead:', error);
		return DEFAULT_SETTINGS;
	}
};
