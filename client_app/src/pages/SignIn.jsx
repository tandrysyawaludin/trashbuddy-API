import React, { Component, Fragment } from 'react';
import {
  Container,
  Row,
  Col,
  Card,
  CardBody,
  Form,
  FormGroup,
  Label,
  Input,
} from 'reactstrap';

import NavbarWelcome from '../partials/NavbarWelcome';

import style from '../styles/SignIn.scss';

class SignIn extends Component {
  render() {
    return <Fragment styleName="SignInPage">
      <NavbarWelcome />
      <Container className="singin_form_container">
        <Row>
          <Col md={{ size: 6, offset: 3 }}>
            <Card>
              <CardBody>
                <Form>
                  <FormGroup>
                    <Label for="exampleEmail">Email</Label>
                    <Input type="email" name="email" placeholder="please input valid email" />
                  </FormGroup>
                  <FormGroup>
                    <Label for="exampleEmail">Password</Label>
                    <Input type="password" name="email" placeholder="input your password" />
                  </FormGroup>
                </Form>
              </CardBody>
            </Card>
          </Col>
        </Row>
      </Container>
    </Fragment>
  }
}

export default SignIn