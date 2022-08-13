import React, { useState, PropsWithChildren } from 'react';
import { IPalette, Tpalette, TmixFunction } from './types';

interface IPaletteContext<T, A> {
	palette?: IPalette<T, A>[];
	mix?: TmixFunction<T, A>;
	theme?: string;

	/** Actions */
	setTheme: (theme: string) => void;
};

interface IPaletteProviderProps<T, A> {
	theme?: string;
}

export const createPaletteContext = <T, A>(palette?: Tpalette<T, A>) => {
	const PaletteContext = React.createContext<IPaletteContext<T, A>>(null);

	const usePalette = () => React.useContext(PaletteContext);

	const PaletteProvider = ({ children, ...props }: PropsWithChildren<IPaletteProviderProps<T, A>>) => {
		let context = React.useContext(PaletteContext);

		const { mix } = palette;
		palette.palette;

		const [theme, updateTheme] = useState(props.theme ?? context?.theme ?? palette[0].name);

		const setTheme = (newTheme: string) => {
			if (context.palette.filter(p => p.name === theme).length > 0)
				updateTheme(newTheme);
		}

		return <PaletteContext.Provider value={{ mix, theme, setTheme }}>
			{children}
		</PaletteContext.Provider>
	}

	return {
		PaletteContext,
		usePalette,
		PaletteProvider,
	};
}