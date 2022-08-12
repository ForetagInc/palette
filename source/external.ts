/** Configuration to create a palette */
export interface IPalette {
	/** Name of the theme */
	name: string;

	/** Colors of the theme */
	colors?: TToken<TToken>;

	colorSchemes?: string[];

	/** Breakpoints of the theme */
	breakpoints?: TToken;

	/** Media Queries of the theme */
	mediaQueries?: TToken;
}

type TToken<T = undefined> = {
	[key: string]: string | T;
};

// Example
const theme: IPalette = {
	name: 'default',
	colors: {
		primary: '#0070f3',
		purple: {
			100: '#faf5ff',
		}
	},
	breakpoints: {
		tablet: '768px',
	}
};