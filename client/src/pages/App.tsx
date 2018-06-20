import * as React from 'react';
import Nav from '../components/Nav.tsx'
import Main from './Main.tsx'

export interface AppProps {
  title: string;
}

const App = () => (
  <div>
    <Nav title={"dkv"} links={["todo:#todo:active"]} />
    <Main />
  </div>
)


export default App;
