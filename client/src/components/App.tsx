import * as React from 'react';
import Nav from './Nav.tsx'

export interface AppProps {
  title: string;
  name: string;
}

function App(props: AppProps) {

  return (
    <div>
      <Nav title={props.title} />
      <br/>
      <div>my name is {props.name}</div>
    </div>
  );
}

export default App;
