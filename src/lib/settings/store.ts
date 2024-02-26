import { Store } from 'tauri-plugin-store-api';
import { deserializeSettings, parseSettingsPayloadJson } from './utils';

export const appStore = new Store('.settings.dat');

export const initAppSettings = async (): Promise<App.Settings> => {
	const res = await appStore.get<string>('settings');

	if (!res) {
		return {
			showSeconds: true,
			opacity: 1,
			clockSize: 50,
			variant: 'Flip',
			pomodoro: {
				showPomodoro: true,
				focusTime: 25,
				interval: 30,
			},
		};
	}

	return deserializeSettings(parseSettingsPayloadJson(res));
};
