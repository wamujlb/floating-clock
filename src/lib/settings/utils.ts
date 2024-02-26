import { z } from 'zod';

export const serializeSettings = (settings: App.Settings): SettingsPayload => ({
	show_seconds: settings.showSeconds,
	opacity: settings.opacity,
	clock_size: settings.clockSize,
	variant: settings.variant,
	pomodoro: {
		show_pomodoro: settings.pomodoro.showPomodoro,
		interval: settings.pomodoro.interval,
		focus_time: settings.pomodoro.focusTime,
	},
});

export const deserializeSettings = (payload: SettingsPayload): App.Settings => ({
	showSeconds: payload.show_seconds,
	opacity: payload.opacity,
	clockSize: payload.clock_size,
	variant: payload.variant,
	pomodoro: {
		showPomodoro: payload.pomodoro.show_pomodoro,
		interval: payload.pomodoro.interval,
		focusTime: payload.pomodoro.focus_time,
	},
});

const settingsPayloadSchema = z.object({
	show_seconds: z.boolean(),
	opacity: z.number(),
	clock_size: z.number(),
	variant: z.union([z.literal('Flip'), z.literal('Digital')]),
	pomodoro: z.object({
		show_pomodoro: z.boolean(),
		focus_time: z.number(),
		interval: z.number(),
	}),
});

export type SettingsPayload = z.infer<typeof settingsPayloadSchema>;

export const parseSettingsPayloadJson = (payload: string): SettingsPayload => {
	return settingsPayloadSchema.parse(JSON.parse(payload));
};
