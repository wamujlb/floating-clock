// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
		interface Settings {
			showSeconds: boolean;
			opacity: number;
			clockSize: number;
		}

		type SettingsChangePayload = {
			show_seconds: boolean;
			opacity: number;
			clock_size: number;
		};
	}
}

export {};
