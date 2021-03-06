import { createContainer } from 'unstated-next';
import { useState, useCallback } from 'react';
import { Map } from 'immutable';
import { BoardOverview } from '../../ts/api/api_trait';

function useSubecribedBoardsState(): {
	subscribed_boards: Map<number, BoardOverview>,
	subscribe: (board: BoardOverview) => void,
	unsubscribe: (id: number) => void,
	load: (boards: BoardOverview[]) => void,
	unload: () => void,
	} {
	let [data, setData] = useState(Map<number, BoardOverview>());
	let subscribe = useCallback((board: BoardOverview): void => {
		setData(m => {
			return m.set(board.id, board);
		});
	}, []);
	let unsubscribe = useCallback((id: number): void => {
		setData(m => {
			return m.remove(id);
		});
	}, []);
	let unload = useCallback((): void => {
		setData(Map());
	}, []);
	let load = useCallback((boards: BoardOverview[]): void => {
		let list: [number, BoardOverview][] = [];
		for (const board of boards) {
			list.push([board.id, board]);
		}
		setData(Map(list));
	}, []);
	return {
		subscribed_boards: data,
		unsubscribe,
		subscribe,
		unload,
		load,
	};
}

export const SubscribedBoardsState = createContainer(useSubecribedBoardsState);