import React, { FC, useState, PropsWithChildren } from 'react';
import { ITheme, Tpalette, TmixFunction } from './types';

interface IPaletteContext<Tokens, Assets, Themes> {
	palette?: ITheme<Tokens, Assets>[];
	mix?: TmixFunction<Tokens, Assets, Themes>;
	theme?: number;

	/** Actions */
	setTheme: (theme: string) => void;
};

interface IPaletteProviderProps {
	theme?: string;
}

const DEFAULT_THEME_INDEX = 0;

export const createPaletteContext = <Tokens, Assets, Themes>({ createMix }: Tpalette<Tokens, Assets, Themes>) => {
	const PaletteContext = React.createContext<IPaletteContext<Tokens, Assets, Themes>>(null);

	const usePalette = () => React.useContext(PaletteContext);

	const PaletteProvider: FC<PropsWithChildren<IPaletteProviderProps>> = ({ children }) => {
		let { theme, palette } = React.useContext(PaletteContext);

		const [currentTheme, updateTheme] = useState(DEFAULT_THEME_INDEX);

		const setTheme = (newTheme: string) => {
			palette.map(({ name }, i) => {
				if (name === newTheme) {
					updateTheme(i);
				}
			});
		}

		return <PaletteContext.Provider value={{ mix: createMix(currentTheme), theme, setTheme }}>
			{children}
		</PaletteContext.Provider>
	}

	return {
		PaletteContext,
		usePalette,
		PaletteProvider,
	};
}