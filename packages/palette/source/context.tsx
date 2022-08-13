import React, { useState, PropsWithChildren } from 'react';
import { IPalette, Tpalette, TmixFunction } from './types';

interface IPaletteContext<T, A> {
	palette?: IPalette<T, A>[];
	mix?: TmixFunction<T, A>;
	theme?: string;

	/** Actions */
	setTheme: (theme: string) => void;
};

interface IPaletteProviderProps {
	theme?: string;
}

const DEFAULT_THEME_INDEX = 0;

export const createPaletteContext = <T, A>({ palette, createMix }: Tpalette<T, A>) => {
	const PaletteContext = React.createContext<IPaletteContext<T, A>>(null);

	const usePalette = () => React.useContext(PaletteContext);

	const PaletteProvider = ({ children }: PropsWithChildren<IPaletteProviderProps>) => {
		let context = React.useContext(PaletteContext);

		const [theme, updateTheme] = useState(DEFAULT_THEME_INDEX);

		const setTheme = (newTheme: string) => {
			context.palette.map(({ name }, i) => {
				if (name === newTheme) {
					updateTheme(i);
				}
			});
		}

		return <PaletteContext.Provider value={{ mix: createMix(theme), theme: palette[theme].name, setTheme }}>
			{children}
		</PaletteContext.Provider>
	}

	return {
		PaletteContext,
		usePalette,
		PaletteProvider,
	};
}