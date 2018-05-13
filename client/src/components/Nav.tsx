import * as React from 'react';

export interface Props {
  title: string;
}

function Nav({title}: Props) {

  return (
    <nav id="nav">
      <div>the title is {title}</div>
    </nav>
  );
}


export default Nav;
