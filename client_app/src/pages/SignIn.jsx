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
  Button
} from 'reactstrap';
import { Link } from 'react-router-dom';
import NavbarWelcome from '../partials/NavbarWelcome';

import '../css/SignIn.css';
class SignIn extends Component {
  submitToLogin = () => {
    this.props.history.push(`/home`)
  }

  render() {
    return (
      <Fragment>
        <NavbarWelcome />
        <Container className="singin-form-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <Card>
                <CardBody>
                  <CardTitle className="text-center">Sign In</CardTitle>
                  <Form>
                    <FormGroup>
                      <Label for="exampleEmail">Email</Label>
                      <Input type="email" name="email" placeholder="please input valid email" autocomplete={false} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Password</Label>
                      <Input type="password" name="email" placeholder="input your password" />
                    </FormGroup>
                    <CardText>
                      <small className="text-muted">Click Sign In button is accept our <CardLink href="#">Terms and Privacy</CardLink></small>
                    </CardText>
                    <FormGroup>
                      <Button color="main" size="md" block onClick={this.submitToLogin}>Sign In</Button>
                    </FormGroup>
                  </Form>
                  <CardText>
                    <small className="text-muted">
                      <Link to="/">About</Link>
                      {} . <Link to="/">Forgot Password?</Link>
                      {} . <Link to="/sign_up">Sign Up</Link>
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

export default SignIn