export const toLocaleDateString = (dateTimeStr: string): string =>
	new Date(dateTimeStr + 'Z').toLocaleDateString();
export const toLocaleTimeString = (dateTimeStr: string): string =>
	new Date(dateTimeStr + 'Z').toLocaleTimeString();
export const serializeDate = (date: Date | null): string | null =>
	date?.toISOString().slice(0, -1) ?? null;
