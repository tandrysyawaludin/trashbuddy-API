import React, { Component, Fragment } from 'react';
import {
  Container,
  Row,
  Col,
  Card,
  CardTitle,
  CardBody,
  CardText,
  CardLink,
  Form,
  FormGroup,
  Label,
  Input,
  Button,
  Alert
} from 'reactstrap';
import { Link } from 'react-router-dom';
import axios from 'axios';

import NavbarWelcome from '../partials/NavbarWelcome';
import loader from '../img/loader.svg';

import '../css/SignUp.css';
class SignUp extends Component {
  constructor(props) {
    super(props);
    this.state = {
      email: "",
      password: "",
      password_1: "",
      full_name: "",
      phone_number: "",
      errorSignUp: false,
      errorSignUpMessage: "",
      submitting: false,
    };

    this.handleInputChange = this.handleInputChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
    this.onDismiss = this.onDismiss.bind(this);
  }

  handleInputChange(event) {
    const value = event.target.value;
    const name = event.target.name;

    this.setState({
      [name]: value
    }, () => {
      if (name === "password" || name === "password_1") {
        (this.state.password !== this.state.password_1) ?
          this.setState({ 
            errorSignUpMessage: "Password and Confirmation Password are not match",
            errorSignUp: true
          }) :
          this.setState({ errorSignUp: false })
      }
    });
  }  

  handleSubmit(event) {
    event.preventDefault();
    event.stopPropagation();

    this.setState({ submitting: true });

    let data = {
      email: this.state.email,
      password: this.state.password,
      full_name: this.state.full_name,
      phone_number: this.state.phone_number,
    }

    let headers = {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    }

    axios({
      method: 'POST',
      url: 'http://localhost:8000/supplier',
      data: data
    })
      .then(response => {
        console.log(response);        
        this.props.history.push('/');
      })
      .catch(error => {
        
      })
      .then(() => {
        this.setState({ submitting: false });  
      });
  }

  onDismiss() {
    this.setState({ errorSignUp: false });
  }

  render() {
    return (
      <Fragment>
        <NavbarWelcome atSignUpPage={true} />
        <Container className="singup-form-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <Alert color="danger" isOpen={this.state.errorSignUp} toggle={this.onDismiss}>
                {this.state.errorSignUpMessage}
              </Alert>
              <Card>
                <CardBody>
                  <CardTitle className="text-center">Sign Up</CardTitle>
                  <Form>
                    <FormGroup>
                      <Label for="exampleEmail">Email</Label>
                      <Input type="email" name="email" placeholder="please input valid email" onChange={this.handleInputChange} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Password</Label>
                      <Input type="password" name="password" placeholder="input your password" onChange={this.handleInputChange} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Confirmation Password</Label>
                      <Input type="password" name="password_1" placeholder="input your password again" onChange={this.handleInputChange} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Full Name</Label>
                      <Input type="text" name="full_name" placeholder="input your full name" onChange={this.handleInputChange} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Phone Number</Label>
                      <Input type="text" name="phone_number" placeholder="input your active phone number" onChange={this.handleInputChange} />
                    </FormGroup>
                    <CardText>
                      <small className="text-muted">Click Sign Up button is accept our <CardLink href="#">Terms and Privacy</CardLink></small>
                    </CardText>
                    <FormGroup>
                      <Button color="main" size="md" block onClick={this.handleSubmit}>
                        {this.state.submitting ? <img src={loader} /> : "Sign Up"}
                      </Button>
                    </FormGroup>
                  </Form>
                  <CardText>
                    <small className="text-muted">
                      <Link to="/">About</Link>
                      {} . <Link to="/">Sign In</Link>
                    </small>
                  </CardText>
                </CardBody>
              </Card>
            </Col>
          </Row>
        </Container>
      </Fragment>
    )
  }
}

export default SignUp