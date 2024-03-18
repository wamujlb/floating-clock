// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}

		type ClockVariant = 'Flip' | 'Digital' | 'BinaryAnalog';
		type PomodoroSettings = {
			showPomodoro: boolean;
			focusTime: number;
			interval: number;
		};

		interface Settings {
			showSeconds: boolean;
			opacity: number;
			clockSize: number;
			variant: ClockVariant;
			pomodoro: PomodoroSettings;
		}
	}

	type Maybe<T> = T | undefined;
}

export {};
