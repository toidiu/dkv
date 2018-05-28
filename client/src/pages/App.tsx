import * as React from 'react';
import Nav from '../components/Nav.tsx'
import Todo from '../components/Todo.tsx'
import styled from 'styled-components';

export interface AppProps {
  title: string;
}

function App(props: AppProps) {

  return (
    <div>
      <Nav title={props.title} links={["github", "about:active"]} />

      <form>
        <Input placeholder="key" type="text" />
      </form>

      <Todo/>
    </div>
  );
}

const Input = styled.input`
  padding: 0.5em;
  margin: 0.5em;
  color: palevioletred;
  background: papayawhip;
  border: none;
  border-radius: 3px;
`;

const Button = styled.button`
  background: palevioletred;
  border-radius: 3px;
  border: none;
  color: white;
`;

export default App;
