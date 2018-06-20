import * as React from 'react';
import styled from 'styled-components';


class Dkv extends React.Component {
  constructor(props) {
    super(props);
    this.state = {value: ''};

    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleChange(event) {
    this.setState({value: event.target.value});
  }

  handleSubmit(event) {
    console.log('A name was submitted: ' + this.state.value);

    event.preventDefault();
  }

  render() {
    return (
      <div>
        <div>dkv</div>

        <form onSubmit={this.handleSubmit}>
          <label>
            Name:
            <input type="text" value={this.state.value} onChange={this.handleChange} />
          </label>
          <input type="submit" value="Submit" />
        </form>
      </div>
    );
  }
}

/* const Input = styled.input` */
/*   padding: 0.5em; */
/*   margin: 0.5em; */
/*   color: palevioletred; */
/*   background: papayawhip; */
/*   border: none; */
/*   border-radius: 3px; */
/* `; */

/* const Button = styled.button` */
/*   background: palevioletred; */
/*   border-radius: 3px; */
/*   border: none; */
/*   color: white; */
/* `; */


export default Dkv;
