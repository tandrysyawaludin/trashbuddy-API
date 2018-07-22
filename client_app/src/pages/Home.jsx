import React, { Component, Fragment } from 'react';
import {
  Container,
  Row,
  Col,
  Button,
  FormGroup,
  Label,
  Input
} from 'reactstrap';
import { Link } from 'react-router-dom';
import CssModules from 'react-css-modules';

import NavbarBottom from '../partials/NavbarBottom';
import NavbarMain from '../partials/NavbarMain';
import styles from '../css/Home.css';

class Home extends Component {
  constructor(props) {
    super(props)
    this.state = {
      category: 1,
      area: 1
    }
  }

  changeArea = (e) => {
    this.setState({ area: e.target.value })
  }

  changeCategory = (e) => {
    this.setState({ category: e.target.value })
  }

  submitToSearch = (category, area) => {
    this.props.history.push(`/search?category=${this.state.category}&area=${this.state.area}&page=1`)
  }

  render() {
    const options = ["apple", "mango", "grapes", "melon", "strawberry"].map(function (fruit) {
      return { label: fruit, value: fruit }
    });
    return (
      <div styleName="Home">
        <NavbarMain />
        <Container className="basic-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <FormGroup>
                <Label for="exampleSelect">Category</Label>
                <div className="select-wrapper ">
                  <Input type="select" name="select" id="exampleSelect" onChange={this.changeArea}>
                    <option>1</option>
                    <option>2</option>
                    <option>3</option>
                    <option>4</option>
                    <option>5</option>
                  </Input>
                </div>
              </FormGroup>
              <FormGroup>
                <Label for="exampleSelect">Area</Label>
                <div className="select-wrapper ">
                  <Input type="select" name="select" id="exampleSelect" onChange={this.changeArea}>
                    <option>1</option>
                    <option>2</option>
                    <option>3</option>
                    <option>4</option>
                    <option>5</option>
                  </Input>
                </div>
              </FormGroup>
              <FormGroup>
                <Button color="main" size="md" block onClick={this.submitToSearch}>Find</Button>
              </FormGroup>
            </Col>
          </Row>
        </Container>
        <NavbarBottom currentRoute={this.props.location.pathname} />  
      </div>
    )
  }
}

export default CssModules(Home, styles, { allowMultiple: true });
