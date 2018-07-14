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
import NavbarMain from '../partials/NavbarMain';

import '../css/Home.css';

class SignIn extends Component {
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
    console.log(this.state.area, this.state.category), 
    this.props.history.push(`/search?category=${this.state.category}&area=${this.state.area}&page=1`)
  }

  render() {
    const options = ["apple", "mango", "grapes", "melon", "strawberry"].map(function (fruit) {
      return { label: fruit, value: fruit }
    });
    return (
      <Fragment>
        <NavbarMain />
        <Container className="home-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <FormGroup>
                <Label for="exampleSelect">Category</Label>
                <Input type="select" name="select" id="exampleSelect" onChange={this.changeCategory}>
                  <option>1</option>
                  <option>2</option>
                  <option>3</option>
                  <option>4</option>
                  <option>5</option>
                </Input>
              </FormGroup>
              <FormGroup>
                <Label for="exampleSelect">Area</Label>
                <Input type="select" name="select" id="exampleSelect" onChange={this.changeArea}>
                  <option>1</option>
                  <option>2</option>
                  <option>3</option>
                  <option>4</option>
                  <option>5</option>
                </Input>
              </FormGroup>
              <FormGroup>
                <Button color="main" size="md" block onClick={this.submitToSearch}>Find</Button>
              </FormGroup>
            </Col>
          </Row>
        </Container>
      </Fragment>
    )
  }
}

export default SignIn