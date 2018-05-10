import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { App, Test } from './components/App.tsx'

ReactDOM.render(
  <Test name="apoorv" bla="blaaa">
    middle
  </Test>,
  document.getElementById('app') as HTMLElement
);
