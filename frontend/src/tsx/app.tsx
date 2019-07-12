import * as React from 'react';
import * as ReactDOM from 'react-dom';
import {
	BrowserRouter as Router,
	Switch,
	Route,
	Redirect,
} from 'react-router-dom';
import { toast } from 'react-toastify';

import 'react-toastify/dist/ReactToastify.css?global';
import 'normalize.css?global';
import '../css/layout.css?global';

import { UserState, BottomPanelState } from './global_state';
import { MainContent } from './main_content';
import { RegisterPage } from './register_page';
import { PartyPage } from './party_page';
import { Header } from './header';
import { LeftPanel } from './left_panel';
import { BottomPanel } from './bottom_panel';

// 配置全域提醒
toast.configure({ position: 'bottom-right' });

function App(): JSX.Element {

	function renderContent(): JSX.Element {
		return <Router>
			<Header></Header>
			<div className="other">
				<Switch>
					<Route exact path="/app" render={() => (
						<>
							<LeftPanel></LeftPanel>
							<MainContent></MainContent>
							<BottomPanel></BottomPanel>
						</>
					)} />
					<Route path="/app/register/:invite_code" render={props =>
						<RegisterPage {...props} />
					} />
					<Route path="/app/party" render={props =>
						<PartyPage {...props} />
					} />
					<Redirect to="/app" />
				</Switch>
			</div>
		</Router>;
	}

	return (
		<div className="app">
			<UserState.Provider>
				<BottomPanelState.Provider>
					{renderContent()}
				</BottomPanelState.Provider>
			</UserState.Provider>
		</div>
	);
}

ReactDOM.render(<App />, document.getElementById('root'));
